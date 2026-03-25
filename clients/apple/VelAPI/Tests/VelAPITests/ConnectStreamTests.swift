import XCTest
@testable import VelAPI

final class ConnectStreamTests: XCTestCase {
    func testConnectSSEParserParsesSingleEventFrame() {
        var parser = ConnectSSEParser()
        XCTAssertNil(parser.consume(line: "event: connect_event"))
        XCTAssertNil(parser.consume(line: "data: {\"id\":1}"))
        let frame = parser.consume(line: "")
        XCTAssertEqual(frame, ConnectSSEFrame(eventName: "connect_event", data: "{\"id\":1}"))
    }

    func testConnectSSEParserIgnoresKeepaliveCommentsAndSupportsMultilineData() {
        var parser = ConnectSSEParser()
        XCTAssertNil(parser.consume(line: ": keepalive"))
        XCTAssertNil(parser.consume(line: "data: line one"))
        XCTAssertNil(parser.consume(line: "data: line two"))
        let frame = parser.consume(line: "")
        XCTAssertEqual(frame, ConnectSSEFrame(eventName: nil, data: "line one\nline two"))
    }
}
