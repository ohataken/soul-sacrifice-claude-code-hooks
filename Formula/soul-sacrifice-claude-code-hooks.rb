class SoulSacrificeClaudeCodeHooks < Formula
  desc "My little toolkit for helping Claude Code and Obsidian work together"
  homepage "https://github.com/ohataken/soul-sacrifice-claude-code-hooks"
  version "0.0.1"

  depends_on arch: :arm64
  depends_on :macos

  on_macos do
    on_arm do
      url "https://github.com/ohataken/soul-sacrifice-claude-code-hooks/releases/download/v#{version}/soul-sacrifice-claude-code-hooks-aarch64-apple-darwin.tar.gz"
      sha256 "a6ffe4eb904fc1ee0ed74638835d4b565b25d5f401a5b383ca9e310e756648a2"
    end
  end

  def install
    bin.install "soul-sacrifice-claude-code-hooks"
  end
end
