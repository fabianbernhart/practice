# flutter_application_1

A new Flutter project.

## Getting Started

This project is a starting point for a Flutter application.

A few resources to get you started if this is your first Flutter project:

- [Lab: Write your first Flutter app](https://docs.flutter.dev/get-started/codelab)
- [Cookbook: Useful Flutter samples](https://docs.flutter.dev/cookbook)

For help getting started with Flutter development, view the
[online documentation](https://docs.flutter.dev/), which offers tutorials,
samples, guidance on mobile development, and a full API reference.


## Add another new feature

1. Create new Feature folders
```sh
# example: mkdir -p features/home/{data,domain,presentation}
mkdir -p features/[name]/{data,domain,presentation}
```

2. Create Router Feature Page
```sh
# example: touch features/home/home_page.dart
touch features/home/[name]_page.dart
```

### Clean Architecture + Feature-first (Dart & Flutter style)

Source: [Github Flutter Sample](https://github.com/flutter/samples/tree/main/compass_app/app/lib)

```sh
lib/
├── main.dart
├── app/                   # App-level configurations (theme, routes)
│   ├── app.dart
│   └── router.dart
├── core/                  # Common utilities (constants, exceptions, themes, services) data
│   ├── constants/
│   └── services/
├── features/              # Feature-based structure (recommended) MVVM similar to ui in Flutter example 
│   ├── auth/
│   │   ├── data/
│   │   ├── domain/
│   │   ├── presentation/
│   │   └── auth_page.dart
│   └── home/
│       ├── data/
│       ├── domain/
│       ├── presentation/
│       └── home_page.dart
└── shared/                # Shared widgets, styles, etc.
    └── widgets/

```
