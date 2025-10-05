# Replace src/lib.rs with the finished version
cat > /workspaces/Rust-OS-/Nexis/src/lib.rs <<'EOF'
#![no_std]
#![feature(alloc_error_handler)] // nightly: enables the alloc error handler attribute

// External alloc crate (used by format!, Vec, etc. in no_std)
extern crate alloc;

pub mod alloc_memory; // renamed from alloc to avoid name collision with `alloc` crate
pub mod context;
pub mod interrupts;
pub mod pit;
pub mod kb;
pub mod vga;
pub mod memory;
pub mod task;
pub mod scheduler;
pub mod process;
pub mod syscall;
pub mod syscall_dispatch;
pub mod fs;
pub mod userland;

/// Single alloc error handler for the whole kernel.
/// Remove any other `#[alloc_error_handler]` definitions (e.g., in alloc.rs).
#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    // Use VGA + serial to log the allocation failure, then halt.
    // Avoid using panic! here (unwinding disabled, may not work in no_std).
    crate::vga::vprintln!("Allocation error: {:?}", layout);
    loop {
        core::hint::spin_loop();
    }
}

/// Re-export PhysicalMemoryManager type for convenience.
pub use memory::PhysicalMemoryManager;

/// Global placeholder for the kernel's physical memory manager (PMM).
/// This is an `Option` so it can be initialized at runtime during early boot.
/// Accessing it is unsafe — call only when you're sure it's initialized.
pub static mut PMM: Option<PhysicalMemoryManager> = None;
EOF

# Remove duplicate alloc_error_handler in src/alloc.rs if present
# (this leaves the one in lib.rs above as the single handler)
if [ -f /workspaces/Rust-OS-/Nexis/src/alloc.rs ]; then
  # remove any lines matching alloc_error_handler attribute and its following fn block header
  # (safe-ish simple removal: remove the attribute line only; if a second fn exists you'll still need to keep one)
  sed -i '/#\[\s*alloc_error_handler\s*]/d' /workspaces/Rust-OS-/Nexis/src/alloc.rs
fi

echo "Wrote src/lib.rs and stripped alloc_error_handler from src/alloc.rs (if present)."
echo "Next: ensure other files don't have crate-level attributes (#![no_std]) and that only lib.rs has alloc_error_handler."
