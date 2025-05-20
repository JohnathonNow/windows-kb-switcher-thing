using System;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Text;
using System.Windows.Forms;

namespace KeyboardDeviceTray
{
    static class Program
    {
        [STAThread]
        static void Main()
        {
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Application.Run(new TrayForm());
        }
    }

    public class TrayForm : Form
    {
        private NotifyIcon trayIcon;
        private bool isTvMode = false;

        public TrayForm()
        {
            trayIcon = new NotifyIcon
            {
                Icon = SystemIcons.Application,
                Visible = true,
                Text = "Keyboard Device Tray"
            };

            //RegisterRawInputKeyboard();

            this.ShowInTaskbar = false;
            this.WindowState = FormWindowState.Minimized;
            Console.WriteLine("Starting...");
            this.Load += (s, e) =>
            {
                this.Hide();
                trayIcon.ContextMenuStrip = new ContextMenuStrip();
                trayIcon.ContextMenuStrip.Items.Add("Exit", null, (s, e) => Application.Exit());
                trayIcon.ContextMenuStrip.Items.Add("TV Mode", null, (s, e) => EnterTvMode());
                trayIcon.ContextMenuStrip.Items.Add("Normal Mode", null, (s, e) => ExitTvMode());
            };
        }

        private void EnterTvMode()
        {
            // Logic to enter TV mode
            Console.WriteLine("Entering TV Mode...");
            isTvMode = true;
            trayIcon.Text = "TV Mode";
            trayIcon.Icon = SystemIcons.Information;
            RunBatchScript("tv.bat");
        }

        private void ExitTvMode()
        {
            // Logic to exit TV mode
            Console.WriteLine("Exiting TV Mode...");
            isTvMode = false;
            trayIcon.Text = "Normal Mode";
            trayIcon.Icon = SystemIcons.Application;
            RunBatchScript("normal.bat");
        }

        private void RunBatchScript(string scriptPath)
        {
            ProcessStartInfo startInfo = new ProcessStartInfo
            {
                FileName = "cmd.exe",
                Arguments = $"/c {scriptPath}",
                RedirectStandardOutput = true,
                UseShellExecute = false,
                CreateNoWindow = true
            };

            using (Process process = new Process())
            {
                process.StartInfo = startInfo;
                process.Start();
            }
        }

        protected override void OnHandleCreated(EventArgs e)
        {
            base.OnHandleCreated(e);
            RegisterRawInputKeyboard();
        }

        protected override void WndProc(ref Message m)
        {
            const int WM_INPUT = 0x00FF;
            if (m.Msg == WM_INPUT)
            {
                ProcessRawInput(m.LParam);
            }
            base.WndProc(ref m);
        }

        private void RegisterRawInputKeyboard()
        {
            RAWINPUTDEVICE[] rid = new RAWINPUTDEVICE[1];
            rid[0].usUsagePage = 0x01;
            rid[0].usUsage = 0x06; // Keyboard
            rid[0].dwFlags = 0x00000100; // RIDEV_INPUTSINK
            rid[0].hwndTarget = this.Handle;
            RegisterRawInputDevices(rid, (uint)rid.Length, (uint)Marshal.SizeOf(typeof(RAWINPUTDEVICE)));
        }

        private void ProcessRawInput(IntPtr hRawInput)
        {
            uint dwSize = 0;
            GetRawInputData(hRawInput, 0x10000003, IntPtr.Zero, ref dwSize, (uint)Marshal.SizeOf(typeof(RAWINPUTHEADER)));
            if (dwSize == 0) return;

            IntPtr buffer = Marshal.AllocHGlobal((int)dwSize);

            try
            {
                if (GetRawInputData(hRawInput, 0x10000003, buffer, ref dwSize, (uint)Marshal.SizeOf(typeof(RAWINPUTHEADER))) == dwSize)
                {
                    RAWINPUT raw = (RAWINPUT)Marshal.PtrToStructure(buffer, typeof(RAWINPUT));
                    if (raw.header.dwType == 1) // Keyboard
                    {
                        string deviceName = GetDeviceName(raw.header.hDevice);
                        bool isLogitech = deviceName.StartsWith("\\\\?\\HID#VID_046D&PID_C52B", StringComparison.OrdinalIgnoreCase);
                        Keys key = (Keys)raw.keyboard.VKey;
                        if (raw.keyboard.Message == 0x0100) // WM_KEYDOWN
                        {
                            Console.WriteLine($"Key: {key}, Device: {deviceName} {(isLogitech ? "(Logitech)" : "(Other)")}");
                            if (isLogitech && !isTvMode)
                            {
                                EnterTvMode();
                            }
                            else if (!isLogitech && isTvMode)
                            {
                                ExitTvMode();
                            }
                        }
                    }
                }
            }
            finally
            {
                Marshal.FreeHGlobal(buffer);
            }
        }

        private string GetDeviceName(IntPtr deviceHandle)
        {
            uint size = 0;
            GetRawInputDeviceInfo(deviceHandle, 0x20000007, null, ref size);
            if (size == 0) return "Unknown";

            StringBuilder sb = new StringBuilder((int)size);
            if (GetRawInputDeviceInfo(deviceHandle, 0x20000007, sb, ref size) > 0)
                return sb.ToString();
            return "Unknown";
        }

        protected override void OnFormClosing(FormClosingEventArgs e)
        {
            trayIcon.Visible = false;
            base.OnFormClosing(e);
        }

        // P/Invoke structures and methods
        [StructLayout(LayoutKind.Sequential)]
        struct RAWINPUTDEVICE
        {
            public ushort usUsagePage;
            public ushort usUsage;
            public uint dwFlags;
            public IntPtr hwndTarget;
        }

        [StructLayout(LayoutKind.Sequential)]
        struct RAWINPUTHEADER
        {
            public uint dwType;
            public uint dwSize;
            public IntPtr hDevice;
            public IntPtr wParam;
        }

        [StructLayout(LayoutKind.Sequential)]
        struct RAWKEYBOARD
        {
            public ushort MakeCode;
            public ushort Flags;
            public ushort Reserved;
            public ushort VKey;
            public uint Message;
            public uint ExtraInformation;
        }

        [StructLayout(LayoutKind.Sequential)]
        struct RAWINPUT
        {
            public RAWINPUTHEADER header;
            public RAWKEYBOARD keyboard;
        }

        [DllImport("user32.dll")]
        private static extern uint RegisterRawInputDevices(RAWINPUTDEVICE[] pRawInputDevice, uint uiNumDevices, uint cbSize);

        [DllImport("user32.dll")]
        private static extern uint GetRawInputData(IntPtr hRawInput, uint uiCommand, IntPtr pData, ref uint pcbSize, uint cbSizeHeader);

        [DllImport("user32.dll", CharSet = CharSet.Unicode)]
        private static extern uint GetRawInputDeviceInfo(IntPtr hDevice, uint uiCommand, StringBuilder pData, ref uint pcbSize);
    }
}