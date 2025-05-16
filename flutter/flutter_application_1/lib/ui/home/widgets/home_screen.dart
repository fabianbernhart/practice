import 'package:flutter/material.dart';

import '../view_model/home_view_model.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key, required this.viewModel});

  final HomeViewModel viewModel;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(
        children: [
          Positioned.fill(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: <Widget>[
                Text(
                  viewModel.title,
                  style: Theme.of(context).textTheme.displayLarge,
                ),
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Text(
                      viewModel.description,
                      style: Theme.of(context).textTheme.headlineSmall,
                    ),
                    Text(
                      viewModel.appName,
                      style: TextStyle(
                        color:
                            Theme.of(context).brightness == Brightness.dark
                                ? Colors.white
                                : Colors.black,
                        fontWeight: FontWeight.bold,
                        fontSize: 40,
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
          Positioned(
            width: MediaQuery.of(context).size.width,
            bottom: 20,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: <Widget>[
                Center(
                  child: SizedBox(
                    width: 350,
                    height: 40,
                    child: FilledButton(
                      onPressed: () {},
                      child: Text(viewModel.getStartedText),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
