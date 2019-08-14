mod platform;

use wedge_core::Platform;

use crate::platform::DesktopPlatform;

fn main() {
    let platform = DesktopPlatform::create();
    wedge_core::run_with(platform);
}
