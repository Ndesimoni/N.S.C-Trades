//! Reading the ForexFactory calendar off the web page.
//!
//! Included because it is the calendar most retail forex traders use, with the
//! honest warning that reading a web page breaks without notice.
//!
//! Two things that are not negotiable. Cache the results and do not hammer the
//! site. And treat a failure to read the page as an **error**, never as "no
//! events today".
//!
//! An empty calendar is indistinguishable from a quiet day, so failing quietly
//! here would switch off news blocking entirely while everything looked fine.
