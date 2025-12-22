import SwiftUI
#if os(iOS)
import UIKit
#endif

@main
struct MyApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}

struct ContentView: View {
    @State var name: String = "John"
    @State var quantity: Int = 0
    @State var progress: Float = 0
    @State var isEditing = false
    @State var text: String = "Current Text is: "
    @State var isShowing: Bool = false
    @State var isModal: Bool = false
    var modal: some View {
        Text("Modal")
    }
    var body: some View {
        VStack {
            Text("\(text)")        .border(.green)
            //TextField("input", text: $text) .border(.green)
            //.textFieldStyle(RoundedBorderTextFieldStyle())
            //.padding()
            //.border(.green)
            TextEditor(text: $text)                          .border(.green)
        }.border(.red)
    }
}
