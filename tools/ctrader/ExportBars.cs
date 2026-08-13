// Exports the bars on a cTrader chart to a CSV this project can read.
//
// HOW TO USE IT
//
//   1. In cTrader, open the Automate tab.
//   2. New → Indicator. Call it ExportBars.
//   3. Delete everything in the editor and paste this file in.
//   4. Build (the hammer icon, or F6).
//   5. Open the chart you want — gold, Daily.
//   6. Scroll LEFT until the chart has loaded as far back as you want.
//      cTrader only holds the bars it has actually loaded, so a chart you
//      have not scrolled back on will export a short file.
//   7. Add the indicator to that chart. It writes the file and tells you
//      where, in the Logs tab.
//
//      If it lands somewhere unexpected, set the Folder parameter when you add
//      the instance — an absolute path like /Users/you/Desktop.
//
// WHAT IT WRITES
//
//   time,open,high,low,close
//   2024-08-13 00:00:00,2465.12,2478.90,2461.33,2472.05
//
// Times are converted to UTC before writing. cTrader charts are usually in
// broker server time, which is often two or three hours off UTC — and an
// export in broker time would shift every candle, and with it every level the
// bot draws. Converting here means it is done once, correctly, rather than
// guessed at later.
//
// The newest bar is deliberately left out. While the market is open it is
// still forming: its high and low have not finished happening, and nothing in
// a CSV can say so.

using System;
using System.Text;
using cAlgo.API;

namespace cAlgo
{
    [Indicator(IsOverlay = false, AccessRights = AccessRights.FullAccess)]
    public class ExportBars : Indicator
    {
        [Parameter("Folder", DefaultValue = "")]
        public string Folder { get; set; }

        protected override void Initialize()
        {
            var folder = WhereToWrite();
            var name = string.Format("{0}_{1}.csv", SymbolName, TimeFrame);

            // Fully qualified: cAlgo.API has its own Path and File types, and an
            // unqualified name is ambiguous between the two.
            var path = System.IO.Path.GetFullPath(System.IO.Path.Combine(folder, name));

            // cTrader runs sandboxed, and the folder it starts in may not exist
            // yet. Creating it is cheaper than a crash that only shows up in
            // the Logs tab.
            System.IO.Directory.CreateDirectory(folder);

            var csv = new StringBuilder();
            csv.AppendLine("time,open,high,low,close");

            // Count stops one short on purpose: the newest bar is still open.
            for (var i = 0; i < Bars.Count - 1; i++)
            {
                csv.AppendLine(string.Format(
                    "{0},{1},{2},{3},{4}",
                    Bars.OpenTimes[i].ToUniversalTime().ToString("yyyy-MM-dd HH:mm:ss"),
                    Bars.OpenPrices[i].ToString("F5"),
                    Bars.HighPrices[i].ToString("F5"),
                    Bars.LowPrices[i].ToString("F5"),
                    Bars.ClosePrices[i].ToString("F5")));
            }

            System.IO.File.WriteAllText(path, csv.ToString());

            Print("Wrote {0} bars to {1}", Bars.Count - 1, path);
            Print("Scroll further left and re-add this indicator if you want more history.");
        }

        /// Where the file goes.
        ///
        /// The Folder parameter wins if it is set. Otherwise the Desktop —
        /// worked out from the home directory rather than from
        /// SpecialFolder.DesktopDirectory, which comes back EMPTY inside
        /// cTrader's sandbox and leaves the file somewhere nobody would look.
        private string WhereToWrite()
        {
            // Trimmed. A folder pasted into cTrader's box can carry a trailing
            // newline or space, and the file system will happily make a folder
            // called "Desktop\n" — which then swallows the export inside a
            // nest of directories nobody would ever look in. That happened
            // three times before anyone noticed.
            if (!string.IsNullOrWhiteSpace(Folder))
                return Folder.Trim();

            var home = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);

            if (string.IsNullOrWhiteSpace(home))
                home = Environment.GetEnvironmentVariable("HOME");

            return string.IsNullOrWhiteSpace(home)
                ? "."
                : System.IO.Path.Combine(home, "Desktop");
        }

        public override void Calculate(int index)
        {
        }
    }
}
