import 'package:flutter/material.dart';

import '../../../routing/routes.dart';

class HomeViewModel extends ChangeNotifier {
  String title = 'Get Started';
  String description = 'Welcome to ';

  String appName = 'Burple!';

  String getStartedText = 'Get Started';

  String copyRoute = Routes.copy;
}
