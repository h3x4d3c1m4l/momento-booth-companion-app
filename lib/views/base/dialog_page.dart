import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

final class DialogPage<T> extends CustomTransitionPage<void> {

  static const defaultTransitionDuration = Duration(milliseconds: 800);

  static CurvedAnimation _fadeAndScaleAnimation(Animation<double> parent) {
    return CurvedAnimation(
      parent: parent,
      curve: Curves.elasticInOut,
      reverseCurve: Curves.elasticIn,
    );
  }

  static CurvedAnimation _blurAnimation(Animation<double> parent) {
    return CurvedAnimation(
      parent: parent,
      curve: Curves.easeOutQuint,
      reverseCurve: Curves.easeInExpo,
    );
  }

  DialogPage({
    required super.key,
    required super.child,
    super.barrierDismissible = false,
  }) : super(
          opaque: false,
          transitionDuration: defaultTransitionDuration,
          reverseTransitionDuration: defaultTransitionDuration,
          transitionsBuilder: (context, animation, secondaryAnimation, child) {
            double blur = _blurAnimation(animation).value * 5;
            return BackdropFilter(
              filter: ImageFilter.blur(sigmaX: blur, sigmaY: blur),
              child: FadeTransition(
                opacity: Tween<double>(begin: 0.0, end: 1.0).animate(_fadeAndScaleAnimation(animation)),
                child: ScaleTransition(
                  scale: Tween<double>(begin: 0.0, end: 1.0).animate(_fadeAndScaleAnimation(animation)),
                  child: FadeTransition(
                    opacity: Tween<double>(begin: 1.0, end: 0.0).animate(_fadeAndScaleAnimation(secondaryAnimation)),
                    child: ScaleTransition(
                      scale: Tween<double>(begin: 1.0, end: 0.0).animate(_fadeAndScaleAnimation(secondaryAnimation)),
                      child: child,
                    ),
                  ),
                ),
              ),
            );
          },
        );

  @override
  Route<T> createRoute(BuildContext context) => RawDialogRoute<T>(
        settings: this,
        barrierColor: barrierColor,
        barrierDismissible: barrierDismissible,
        barrierLabel: barrierLabel,
        pageBuilder: (context, animation, secondaryAnimation) => child,
        transitionBuilder: transitionsBuilder,
        transitionDuration: transitionDuration,
      );

}
