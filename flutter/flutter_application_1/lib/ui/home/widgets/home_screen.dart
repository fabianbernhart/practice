import 'package:flutter/material.dart';

import '../view_model/home_view_model.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key, required this.viewModel});

  final HomeViewModel viewModel;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text(viewModel.title)),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text(
              viewModel.title,
              style: Theme.of(context).textTheme.displayLarge,
            ),
            Text(
              viewModel.description,
              style: Theme.of(context).textTheme.headlineSmall,
            ),
            FilledButton(
              onPressed: () {},
              child: Text(viewModel.getStartedText),
            ),
          ],
        ),
      ),
    );
  }
}
