import SwiftUI
import UIKit


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
    var body: some View {
        VStack {
            Text("Steppr \(quantity)")
            Text("Slider \(progress)")
            Button(
                action: {
                    print("BUTTON PRESSED");
                    // did tap
                },
                label: { Text("Click Me") }
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
            Slider(value: $progress, in: 0...100)
        }
        //.padding()
    }
}
