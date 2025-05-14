import 'package:flutter/material.dart';

import '../../../routing/routes.dart';

class HomeViewModel extends ChangeNotifier {
  String title = 'Home';
  String description = 'Welcome to the Home Screen!';

  String getStartedText = 'Get Started';

  String copyRoute = Routes.copy;
}
