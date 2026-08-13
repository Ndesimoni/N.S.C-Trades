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
//      where, in the Log tab.
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
            var folder = string.IsNullOrWhiteSpace(Folder)
                ? Environment.GetFolderPath(Environment.SpecialFolder.DesktopDirectory)
                : Folder;

            var name = string.Format("{0}_{1}.csv", SymbolName, TimeFrame);
            // Fully qualified: cAlgo.API has its own Path and File types, and an
            // unqualified name is ambiguous between the two.
            var path = System.IO.Path.Combine(folder, name);

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

        public override void Calculate(int index)
        {
        }
    }
}
