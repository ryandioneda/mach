#include "hooks/keyboardHook.h"
#include <debugapi.h>
#include <windows.h>
#include <winuser.h>

KeyboardHook::KeyboardHook() : hookHandle_(nullptr) {
  hookHandle_ = SetWindowsHookExW(WH_KEYBOARD_LL, LowLevelKeyboardProc,
                                  GetModuleHandleW(nullptr), 0);
  if (!hookHandle_) {
    OutputDebugStringW(L"[HOOK] Failed to install hook");
  }
}

KeyboardHook::~KeyboardHook() {
  if (hookHandle_) {
    UnhookWindowsHookEx(hookHandle_);
    hookHandle_ = nullptr;
  }
}

LRESULT CALLBACK KeyboardHook::LowLevelKeyboardProc(int nCode, WPARAM wParam,
                                                    LPARAM lParam) {
  if (nCode < 0) {
    return CallNextHookEx(nullptr, nCode, wParam, lParam);
  }
  KBDLLHOOKSTRUCT *kb{reinterpret_cast<KBDLLHOOKSTRUCT *>(lParam)};
  OutputDebugStringW(L"[HOOK] LowLevelKeyboardProc");

  return CallNextHookEx(nullptr, nCode, wParam, lParam);
}
