#include "hooks/keyboardHook.h"
#include <debugapi.h>
#include <minwindef.h>
#include <windows.h>
#include <winuser.h>

int WINAPI wWinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance,
                    LPWSTR lpCmdLine, int nShowCmd) {
  OutputDebugStringW(L"[MAIN] Hello, World!");
  KeyboardHook KeyboardHook;
  OutputDebugStringW(L"[MAIN] Installed Keyboard hook");

  // TODO: 1. register window class for main window

  // TODO: 2. create the window

  // TODO: 3. show the window

  MSG msg;
  BOOL bRet;

  while ((bRet = GetMessage(&msg, NULL, 0, 0)) != 0) {
    if (bRet == -1) {

      // FIX: Properly handle the error
      OutputDebugStringW(L"[MAIN] bRet is -1. Handle Error");
    } else {
      TranslateMessage(&msg);
      DispatchMessage(&msg);
    }
  }

  return msg.wParam;
}
