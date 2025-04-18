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
        //    Spacer()
            Text("\(text) - switch is \(isShowing)")        .border(.green)
            Image(systemName: "clock")                      .border(.green)
            Toggle(isOn: $isShowing) {}                     .border(.green)
            //.background(Color.purple)
            TextField("input", text: $text)
            .border(.green)
            .textFieldStyle(RoundedBorderTextFieldStyle())
            .padding()
                                                            .border(.green)
            /*
            Button("Modal") {
                self.isModal = true
            }.sheet(isPresented: $isModal, content: {
                    self.modal
                    }).border(.green)
            Image(systemName: "cloud.heavyrain.fill")
            .foregroundColor(.red)
            .font(.title)
            */
        }.border(.red)
        /*
        VStack {
            Text("Stepper \(quantity)")
            Button(
                action: {
                    print("BUTTON PRESSED");
                },
                label: { Text("Click Me \(name)") }
            )
            Button(
                action: {
                }, label: {
                    Image(systemName: "clock")
                    Text("Click Me")
                    Text("Hello World")
                }
            )
            .foregroundColor(Color.white)
            .padding()
            .background(Color.blue)
            .cornerRadius(5)

            TextField("Name's placeholder", text: $name)
            .textFieldStyle(RoundedBorderTextFieldStyle())
            .padding()

            Stepper(value: $quantity, in: 0...10, label: { Text("Quantity \(quantity)")})
            Slider(
                value: $progress,
                in: 0...100,
                step: 5,

                onEditingChanged: { editing in
                    isEditing = editing
                }
            )
            Text("Slider \(progress)")
                .foregroundColor(isEditing ? .red : .blue)
        }
        //.padding()
        */
        /*
        */
    }
}
