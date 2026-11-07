plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "net.cawet.luxtoncrane"
    compileSdk = 36

    defaultConfig {
        applicationId = "net.cawet.luxtoncrane"
        minSdk = 26
        targetSdk = 36
        versionCode = 1
        versionName = "0.1.0"
        ndk {
            abiFilters += "arm64-v8a"
        }
    }

    // La clé fixe n'existe que dans la livraison : une mise à jour n'est acceptée par Android
    // que si l'APK porte la même signature que l'application installée.
    val signingKeyPath = System.getenv("LIVRAISON_CLE_CHEMIN")
    if (signingKeyPath != null) {
        signingConfigs {
            create("livraison") {
                storeFile = file(signingKeyPath)
                storePassword = System.getenv("LIVRAISON_CLE_MOT_DE_PASSE")
                keyAlias = "luxtoncrane"
                keyPassword = System.getenv("LIVRAISON_CLE_MOT_DE_PASSE")
            }
        }
        buildTypes {
            getByName("release") {
                signingConfig = signingConfigs.getByName("livraison")
            }
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    sourceSets["main"].java.srcDir(layout.buildDirectory.dir("generated/pont/kotlin"))
    sourceSets["main"].jniLibs.srcDir(layout.buildDirectory.dir("generated/pont/jniLibs"))
}

kotlin {
    jvmToolchain(17)
}

dependencies {
    // Le code Kotlin généré par UniFFI charge la bibliothèque Rust avec JNA.
    implementation("net.java.dev.jna:jna:5.19.1@aar")
    // La seule façon fiable de lancer une tâche périodique en arrière-plan, Doze et redémarrages compris.
    implementation("androidx.work:work-runtime:2.12.0")
    testImplementation(kotlin("test"))
}

// Le métier en Rust est compilé pour Android, puis son code Kotlin d'appel est généré par UniFFI.
val cargoWorkspace = rootDir.parentFile
val generatedJniLibs = layout.buildDirectory.dir("generated/pont/jniLibs")
val generatedKotlin = layout.buildDirectory.dir("generated/pont/kotlin")

val buildRustBridge by tasks.registering(Exec::class) {
    workingDir = cargoWorkspace
    commandLine(
        "cargo", "ndk",
        "--target", "arm64-v8a",
        "--output-dir", generatedJniLibs.get().asFile.path,
        "build", "--release", "--package", "pont",
    )
}

val generateRustBridgeKotlin by tasks.registering(Exec::class) {
    dependsOn(buildRustBridge)
    workingDir = cargoWorkspace
    commandLine(
        "cargo", "run", "--package", "pont", "--features", "bindgen", "--bin", "uniffi-bindgen", "--",
        "generate",
        "--library", "target/aarch64-linux-android/release/libpont.so",
        "--language", "kotlin",
        "--no-format",
        "--out-dir", generatedKotlin.get().asFile.path,
    )
}

tasks.named("preBuild") {
    dependsOn(generateRustBridgeKotlin)
}
