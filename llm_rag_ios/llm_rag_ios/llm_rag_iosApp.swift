//
//  llm_rag_iosApp.swift
//  llm_rag_ios
//
//  Created by onihilist on 7/24/25.
//

import SwiftUI

@main
struct llm_rag_iosApp: App {
    let persistenceController = PersistenceController.shared

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environment(\.managedObjectContext, persistenceController.container.viewContext)
        }
    }
}
