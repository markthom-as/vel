import ActivityKit
import SwiftUI
import WidgetKit

struct VelStatusEntry: TimelineEntry {
    let date: Date
    let headline: String
    let detail: String
}

struct VelStatusProvider: TimelineProvider {
    func placeholder(in context: Context) -> VelStatusEntry {
        VelStatusEntry(date: Date(), headline: "Vel", detail: "Open commitments and nudges")
    }

    func getSnapshot(in context: Context, completion: @escaping (VelStatusEntry) -> Void) {
        completion(placeholder(in: context))
    }

    func getTimeline(in context: Context, completion: @escaping (Timeline<VelStatusEntry>) -> Void) {
        let now = Date()
        let entry = VelStatusEntry(
            date: now,
            headline: "Vel",
            detail: "Widget scaffold ready"
        )
        completion(Timeline(entries: [entry], policy: .after(now.addingTimeInterval(15 * 60))))
    }
}

struct VelStatusWidgetEntryView: View {
    var entry: VelStatusProvider.Entry

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(entry.headline)
                .font(.headline)
            Text(entry.detail)
                .font(.caption)
                .foregroundStyle(.secondary)
                .lineLimit(3)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
        .padding(8)
    }
}

struct VelStatusWidget: Widget {
    let kind: String = "VelStatusWidget"

    var body: some WidgetConfiguration {
        StaticConfiguration(kind: kind, provider: VelStatusProvider()) { entry in
            VelStatusWidgetEntryView(entry: entry)
        }
        .configurationDisplayName("Vel Status")
        .description("Glanceable now-state and complication-ready surfaces.")
        .supportedFamilies([
            .systemSmall,
            .systemMedium,
            .accessoryInline,
            .accessoryCircular,
            .accessoryRectangular
        ])
    }
}

@available(iOSApplicationExtension 16.1, *)
struct VelFocusAttributes: ActivityAttributes {
    public struct ContentState: Codable, Hashable {
        var detail: String
    }

    var title: String
}

@available(iOSApplicationExtension 16.1, *)
struct VelLiveActivityWidget: Widget {
    var body: some WidgetConfiguration {
        ActivityConfiguration(for: VelFocusAttributes.self) { context in
            VStack(alignment: .leading, spacing: 6) {
                Text(context.attributes.title)
                    .font(.headline)
                Text(context.state.detail)
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
            .padding(10)
        } dynamicIsland: { context in
            DynamicIsland {
                DynamicIslandExpandedRegion(.center) {
                    Text(context.state.detail)
                        .font(.caption)
                }
            } compactLeading: {
                Text("Vel")
            } compactTrailing: {
                Image(systemName: "bolt.fill")
            } minimal: {
                Image(systemName: "bolt.fill")
            }
        }
    }
}

@main
struct VelWidgetExtensionBundle: WidgetBundle {
    @WidgetBundleBuilder
    var body: some Widget {
        VelStatusWidget()
        if #available(iOSApplicationExtension 16.1, *) {
            VelLiveActivityWidget()
        }
    }
}
