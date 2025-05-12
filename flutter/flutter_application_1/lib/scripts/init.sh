#!/bin/bash
FEATURE_NAME="home" # Default feature name if not provided

mkdir -p ui/core/ui
mkdir -p ui/core/themes
mkdir -p ui/${FEATURE_NAME}/view_model
mkdir -p ui/${FEATURE_NAME}/widgets
mkdir -p domain/models
mkdir -p data/repositories
mkdir -p data/services
mkdir -p data/model
mkdir -p config
mkdir -p utils
mkdir -p routing

touch main.dart main_staging.dart main_development.dart

echo "// Shared widgets go here" > ui/core/ui/shared_widgets.dart
echo "// Theme files go here" > ui/core/themes/theme.dart
echo "// ViewModel class for $FEATURE_NAME" > ui/${FEATURE_NAME}/view_model/${FEATURE_NAME}_view_model.dart
echo "// Main screen for $FEATURE_NAME" > ui/${FEATURE_NAME}/widgets/${FEATURE_NAME}_screen.dart

echo "// Domain model for example" > domain/models/sample_model.dart
echo "// Repository class" > data/repositories/sample_repository.dart
echo "// Service class" > data/services/sample_service.dart
echo "// API model class" > data/model/api_model.dart
