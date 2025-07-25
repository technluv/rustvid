# Homebrew Formula for Rust Video Editor
# https://github.com/your-org/rust-video-editor

class RustVideoEditor < Formula
  desc "High-performance video editor built with Rust, featuring real-time effects and GPU acceleration"
  homepage "https://github.com/your-org/rust-video-editor"
  url "https://github.com/your-org/rust-video-editor/archive/v1.0.0.tar.gz"
  sha256 "placeholder_sha256_hash_here"
  license "MIT"
  head "https://github.com/your-org/rust-video-editor.git", branch: "main"

  depends_on "rust" => :build
  depends_on "pkg-config" => :build
  depends_on "cmake" => :build
  depends_on "ffmpeg"
  depends_on "gtk+3"
  depends_on "gstreamer"
  depends_on "gst-plugins-base"
  depends_on "gst-plugins-good"
  depends_on "gst-plugins-bad"
  depends_on "gst-plugins-ugly"
  depends_on "opencv"
  depends_on "vulkan-loader"

  def install
    system "cargo", "build", "--release", "--locked"
    bin.install "target/release/rust-video-editor"
    
    # Install desktop file
    share.install "assets/rust-video-editor.desktop"
    
    # Install icons
    icons_dir = share/"icons/hicolor"
    %w[16 32 48 64 128 256].each do |size|
      (icons_dir/"#{size}x#{size}/apps").mkpath
      (icons_dir/"#{size}x#{size}/apps").install "assets/icons/icon-#{size}.png" => "rust-video-editor.png"
    end
    
    # Install man page
    man1.install "docs/rust-video-editor.1"
  end

  test do
    system "#{bin}/rust-video-editor", "--version"
    assert_match "Rust Video Editor", shell_output("#{bin}/rust-video-editor --version")
  end
end