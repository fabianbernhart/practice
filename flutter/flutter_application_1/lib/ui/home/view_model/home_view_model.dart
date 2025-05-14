import 'package:flutter/material.dart';

import '../../../routing/routes.dart';

class HomeViewModel extends ChangeNotifier {
  String title = 'Get Started';
  String description = 'Welcome to Burple!';

  String getStartedText = 'Get Started';

  String copyRoute = Routes.copy;
}
