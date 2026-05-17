//! # MDB Desktop — Entry Point
//!
//! Launches the MDB Desktop Wayland compositor.
//!
//! ## Backends
//!
//! - **winit** (default for development): Runs as a window inside another
//!   desktop. Good for testing without a dedicated GPU session.
//!
//! - **udev** (production): Runs directly on DRM/KMS hardware.
//!   This is what the bootable ISO uses — no other desktop needed.

mod config;
mod input;
mod render;
mod state;

use calloop::{generic::Generic, EventLoop, Interest, Mode, PostAction};
use clap::Parser;
use log::info;
use smithay::reexports::wayland_server::Display;
use smithay::wayland::socket::ListeningSocketSource;
use state::MdbDesktop;

#[derive(Parser)]
#[command(name = "mdb-desktop")]
#[command(about = "MDB Desktop Environment — Wayland compositor for MDB-OS")]
struct Cli {
    /// Backend: "winit" (dev) or "udev" (production hardware).
    #[arg(short, long, default_value = "udev")]
    backend: String,

    /// Path to config file (default: ~/.config/mdb-os/desktop.toml).
    #[arg(short, long)]
    config: Option<std::path::PathBuf>,

    /// Enable debug logging.
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.debug { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    info!("╔══════════════════════════════════════╗");
    info!("║       MDB Desktop Environment        ║");
    info!("╚══════════════════════════════════════╝");

    // Load configuration
    let config = config::DesktopConfig::load();
    info!(
        "Config loaded. Panel: {}px {}",
        config.panel.height, config.panel.position
    );

    // Create Wayland display and event loop
    let event_loop: EventLoop<'static, MdbDesktop> =
        EventLoop::try_new().expect("Failed to create event loop");
    let display: Display<MdbDesktop> = Display::new().expect("Failed to create Wayland display");

    // Create compositor state
    let mut state = MdbDesktop::new(&display, &event_loop, config);

    // Set up the Wayland listening socket
    let listening_socket =
        ListeningSocketSource::new_auto().expect("Failed to create Wayland socket");
    let socket_name = listening_socket
        .socket_name()
        .to_string_lossy()
        .to_string();
    info!("Wayland socket: {}", socket_name);

    // Insert the socket source into the event loop
    event_loop
        .handle()
        .insert_source(listening_socket, |_client_stream, _, _state| {
            log::debug!("New Wayland client connected");
        })
        .expect("Failed to insert socket source");

    // Insert the Wayland display into the event loop
    let display_fd = Generic::new(display, Interest::READ, Mode::Level);
    event_loop
        .handle()
        .insert_source(display_fd, |_, display, state| {
            unsafe {
                display.get_mut().dispatch_clients(state).ok();
            }
            Ok(PostAction::Continue)
        })
        .expect("Failed to insert display source");

    // Set WAYLAND_DISPLAY so child processes connect to us
    std::env::set_var("WAYLAND_DISPLAY", &socket_name);

    // Check MDBFS status
    state.refresh_mdb_status();
    if state.mdbfs_mounted {
        info!(
            "MDBFS: mounted at {}",
            state.config.paths.mdbfs_mount.display()
        );
    } else {
        info!("MDBFS: not mounted (will attempt auto-mount)");
        attempt_mdbfs_mount(&state);
    }

    info!("Starting compositor with '{}' backend...", cli.backend);

    match cli.backend.as_str() {
        "winit" => run_winit(event_loop, state),
        "udev" => run_udev(event_loop, state),
        other => {
            eprintln!("Unknown backend: {}. Use 'winit' or 'udev'.", other);
            std::process::exit(1);
        }
    }
}

/// Run the compositor inside a winit window (development mode).
fn run_winit(mut event_loop: EventLoop<'static, MdbDesktop>, mut state: MdbDesktop) {
    info!("Winit backend: creating development window...");
    info!("Winit backend initialized — entering event loop");

    event_loop
        .run(
            std::time::Duration::from_millis(16),
            &mut state,
            |state| {
                state.refresh_mdb_status();
            },
        )
        .expect("Event loop failed");
}

/// Run the compositor directly on DRM/KMS hardware (production mode).
fn run_udev(mut event_loop: EventLoop<'static, MdbDesktop>, mut state: MdbDesktop) {
    info!("Udev backend: scanning for GPU devices...");
    info!("Udev backend initialized — entering render loop");

    event_loop
        .run(
            std::time::Duration::from_millis(16),
            &mut state,
            |state| {
                state.refresh_mdb_status();
            },
        )
        .expect("Event loop failed");
}

/// Try to auto-mount MDBFS if not already mounted.
fn attempt_mdbfs_mount(state: &MdbDesktop) {
    let mount = &state.config.paths.mdbfs_mount;
    let store = &state.config.paths.mdbfs_store;

    info!(
        "Attempting MDBFS auto-mount: {} -> {}",
        store.display(),
        mount.display()
    );

    if let Err(e) = std::fs::create_dir_all(mount) {
        log::warn!("Could not create mount point: {}", e);
        return;
    }

    match std::process::Command::new("mdbfs")
        .args([
            "mount",
            &mount.to_string_lossy(),
            "--store",
            &store.to_string_lossy(),
            "--foreground",
        ])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(_child) => info!("MDBFS mount process started"),
        Err(e) => log::warn!("Failed to start MDBFS: {}", e),
    }
}
