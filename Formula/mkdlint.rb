class Mkdlint < Formula
  desc "Fast Markdown linter written in Rust"
  homepage "https://github.com/192d-Wing/mkdlint"
  license "Apache-2.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/192d-Wing/mkdlint/releases/latest/download/mkdlint-macos-aarch64.tar.gz"
    else
      url "https://github.com/192d-Wing/mkdlint/releases/latest/download/mkdlint-macos-x86_64.tar.gz"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/192d-Wing/mkdlint/releases/latest/download/mkdlint-linux-aarch64.tar.gz"
    else
      url "https://github.com/192d-Wing/mkdlint/releases/latest/download/mkdlint-linux-x86_64.tar.gz"
    end
  end

  def install
    bin.install "mkdlint"
    bin.install "mkdlint-lsp"
  end

  test do
    (testpath/"test.md").write("# Hello\n\nWorld\n")
    system bin/"mkdlint", "test.md"
  end
end
