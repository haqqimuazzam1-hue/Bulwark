CHANGELOG
All notable changes to this project will be documented in this file.
The format is based on Keep a Changelog,
and this project follows Semantic Versioning.

[0.2.0] – Decision Engine & Inspector Refactor
Release date: 7 February 2026

✨ Added
• Centralized Decision Engine (Allow / Log / Block)
• End-to-end security decision pipeline
• InspectorFinding model with severity levels:
  • Low
  • Medium
  • High
•Server executor that applies security decisions
•End-to-end working example (basic_api)

🔄 Changed
• Inspector architecture refactored:
  • Inspectors no longer return decisions
  • Inspectors only report findings
• Logging system now follows final decision only
• Request handling standardized via normalized context
• Public API adjusted to support decision-first design

🛡️ Security
• HTTP Method inspection (whitelist-based)
• Header size inspection with soft & hard limits
• User-Agent heuristic inspection
• Decision escalation based on inspector severity

🧹 Removed
• Legacy inspector patterns that returned decisions directly
• Implicit decision logic inside inspectors
• Old example APIs incompatible with the new architecture

⚠️ Notes
• This release introduces breaking internal changes
• Public API may still evolve before v1.0.0
• Recommended for experimentation and feedback

[0.1.0] – Initial Release
Release date: 5 February 2026

✨ Added
• Initial project structure
• Request context & normalization
• Basic inspector prototypes
• Simple logging
• Early framework skeleton

⚠️ Notes
• Proof of concept
• No centralized decision logic
• API not stable
