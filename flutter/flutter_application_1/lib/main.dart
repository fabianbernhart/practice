import 'package:flutter/material.dart';

import 'routing/router.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});
  @override
  Widget build(BuildContext context) {
    ThemeData theme = ThemeData(
      colorScheme: ColorScheme.fromSeed(
        seedColor: Colors.deepPurple,
        brightness: Brightness.light,
      ),
    );

    ThemeData darkTheme = ThemeData(
      colorScheme: ColorScheme.fromSeed(
        brightness: Brightness.dark,
        seedColor: Colors.deepPurpleAccent,
      ),
    );

    return MaterialApp.router(
      title: 'Flutter Demo',
      themeMode: ThemeMode.system,
      theme: theme,
      darkTheme: darkTheme,
      routerConfig: router(),
    );
  }
}
