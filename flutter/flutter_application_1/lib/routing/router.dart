import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import '../ui/home/view_model/home_view_model.dart';
import '../ui/home/widgets/home_screen.dart';
import 'routes.dart';

GoRouter router() => GoRouter(
  initialLocation: Routes.home,
  debugLogDiagnostics: true,
  routes: [
    GoRoute(
      path: Routes.home,
      builder: (context, state) {
        final viewModel = HomeViewModel();
        return HomeScreen(viewModel: viewModel);
      },
    ),
  ],
);
