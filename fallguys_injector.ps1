Add-Type -AssemblyName System.Windows.Forms;
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class WindowHandler {
  [DllImport("user32.dll")]
  public static extern IntPtr GetForegroundWindow();
}
"@

try {
  $ActiveHandle = [WindowHandler]::GetForegroundWindow();
  $fallguys = Get-Process -Name FallGuys_client_game;
  echo $fallguys
  if ($ActiveHandle -ne $fallguys.MainWindowHandle) {
     echo failed
     exit 1
  }
  $command = ""
  switch ($args[0]) {
         "dive"   { $command = "^" }
         "jump"   { $command = " " }
         "up"     { $command = $("wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww" * 20) }
         "down"   { $command = $("ssssssssssssssssssssssssssssssssssssss" * 20) }
         "left"   { $command = $("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" * 20) }
         "right"  { $command = $("dddddddddddddddddddddddddddddddddddddd" * 20) }
         "emote1" { $command = "1" }
         "emote2" { $command = "2" }
         "emote3" { $command = "3" }
         "emote4" { $command = "4" }
        default { exit }
  }
  [System.Windows.Forms.SendKeys]::SendWait($command)
} catch {
}
