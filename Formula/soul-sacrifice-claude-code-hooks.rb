class SoulSacrificeClaudeCodeHooks < Formula
  desc "My little toolkit for helping Claude Code and Obsidian work together"
  homepage "https://github.com/ohataken/soul-sacrifice-claude-code-hooks"
  version "0.0.2"

  depends_on arch: :arm64
  depends_on :macos

  on_macos do
    on_arm do
      url "https://github.com/ohataken/soul-sacrifice-claude-code-hooks/releases/download/v#{version}/soul-sacrifice-claude-code-hooks-aarch64-apple-darwin.tar.gz"
      sha256 "2a93b5316ae872e18ea9362011d078b1313d342c62ea84a2fe806309b4305bbb"
    end
  end

  def install
    bin.install "soul-sacrifice-claude-code-hooks"
  end
end
