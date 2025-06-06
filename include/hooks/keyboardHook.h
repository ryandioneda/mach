#ifndef KEYBOARDHOOK_H
#define KEYBOARDHOOK_H
#include <windows.h>
class KeyboardHook {
public:
  KeyboardHook();
  ~KeyboardHook();

private:
  HHOOK hookHandle_;
  static LRESULT CALLBACK LowLevelKeyboardProc(int nCode, WPARAM wParam,
                                               LPARAM lParam);
};

#endif
