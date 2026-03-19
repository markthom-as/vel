import AppIntents

@main
struct VelIntentsExtension: AppIntentsExtension {}

struct VelQuickCaptureIntent: AppIntent {
    static var title: LocalizedStringResource = "Quick Capture"
    static var description = IntentDescription("Create a quick capture placeholder in Vel.")
    static var openAppWhenRun = true

    @Parameter(title: "Capture text")
    var text: String

    init() {}

    init(text: String) {
        self.text = text
    }

    func perform() async throws -> some IntentResult & ProvidesDialog {
        let trimmed = text.trimmingCharacters(in: .whitespacesAndNewlines)
        let message = trimmed.isEmpty
            ? "Quick capture prepared in Vel."
            : "Quick capture prepared in Vel: \(trimmed)"
        return .result(dialog: .init(stringLiteral: message))
    }
}
