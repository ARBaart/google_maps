use chrono::NaiveDateTime;
use crate::directions::request::Request;

impl<'a> Request<'a> {

    /// Specifies the desired arrival time.
    ///
    /// ## Arguments:
    ///
    /// * `arrival_time` ‧ The time the passenger should arrive at their final
    /// destination by.
    ///
    /// ## Description:
    ///
    /// Specifies the desired time of arrival for _transit_ directions. You can
    /// use either the `.with_departure_time()` or the `.with_arrival_time()`
    /// method, but not both together.
    ///
    /// ## Example:
    ///
    /// * Arriving by January 1, 2019 at 12:00:00 AM:
    /// ```
    /// .with_arrival_time(NaiveDate::from_ymd(2019, 1, 1).and_hms(0, 00, 0))
    /// ```

    pub fn with_arrival_time(&'a mut self, arrival_time: NaiveDateTime) -> &'a mut Request {
        self.arrival_time = Some(arrival_time);
        self
    } // fn

} // impl