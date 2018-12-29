var days = [
  "Sunday",
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday"
];

var table;
var hidden_times;
var times;
var countdownElement;

function getDaysHoursMinutes(distance) {
  var days = Math.floor(distance / (1000 * 60 * 60 * 24));
  var hours = Math.floor((distance % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
  var minutes = Math.floor((distance % (1000 * 60 * 60)) / (1000 * 60));
  return [days, hours, minutes];
}

function setCountdownTimer() {
  let nextSession = getNextSession();
  let now = new Date();
  if (nextSession !== null) {
    let timeCell = nextSession.cells[2];
    let sessioName = nextSession.cells[0].textContent;
    let date = new Date(timeCell.textContent);
    if ((now - date) < 0) {
      var distance = date - now;
      [days, hours, minutes] = getDaysHoursMinutes(distance);
      countdownElement.innerHTML = `${sessioName} in ${days}d ${hours}h ${minutes}m`;
      var x = setInterval(function() {
        var now = new Date().getTime();
        var distance = date - now;
        [days, hours, minutes] = getDaysHoursMinutes(distance);
        countdownElement.innerHTML = `${sessioName} ${days}d ${hours}h ${minutes}m`;

        if (distance < 0) {
          clearInterval(x);
          countdownElement.innerHTML = "";
        }
      }, 60000);
    }
  } else {
    countdownElement.innerHTML = "";
  }
}

// Return Row of next upcoming session
// returns null if there are no sessions or
// if all sessions are in the past
function getNextSession() {
  let now = new Date();
  for (var time of hidden_times) {
    let date = new Date(time.textContent);
    if ((now - date) < 0) {
      return time.parentElement;
    }
  }
  return null;
}

function updateTimes() {
  var index = 0;
  var lastTimezoneOffset = null;
  for (var time of hidden_times) {
    var date = new Date(time.textContent);
    var timezoneOffset = date.getTimezoneOffset();

    if (lastTimezoneOffset !== null) {
      if (lastTimezoneOffset !== timezoneOffset) {
        addDstWarningRow(index + 1);
      }
    }
    lastTimezoneOffset = timezoneOffset;

    var day = days[date.getDay()];
    var hours = date.getHours().toString().padStart(2, '0');
    var minutes = date.getMinutes().toString().padStart(2, '0');

    var timeString = `${day}, ${hours}:${minutes}`;
    times[index].textContent = timeString;
    index += 1;
  }
}

function addDstWarningRow(index) {
  var row = table.insertRow(index);
  row.style.height = "10px";
  row.style.backgroundColor = "red";
  row.style.color = "black";
  row.style.fontWeight = "bold";

  var cell = row.insertCell(0);
  cell.innerHTML = "Warning DST change!!";
  cell.colSpan = 3;
}

document.addEventListener('DOMContentLoaded', function () {
  hidden_times = document.getElementsByClassName("hidden-time");
  times = document.getElementsByClassName("session-time");
  table = document.getElementsByClassName("sessions_table")[0];
  countdownElement = document.getElementById("next-session-countdown");
  updateTimes();
  setCountdownTimer();
});
