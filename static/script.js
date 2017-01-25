
function get_event_date_range(e) {
  if (e.sessions.length === 0) {
    return "TBD";
  } else if (e.sessions.length === 1) {
    return e.sessions[0].date;
  } else {
    var start = e.sessions[0].date;
    var len = e.sessions.length;
    var end = e.sessions[len-1].date;
    return start + ", " + end;
  }
}

function toggle_button_clicked(button, sport) {
  var class_list = button.classList;

  // If it's on turn it off, and hide it's events
  if (in_array(class_list, 'filter-button-on')) {
    button.classList.add("filter-button-off");
    button.classList.remove("filter-button-on");
    filter_table(sport, true);
  } else {
    button.classList.remove("filter-button-off");
    button.classList.add("filter-button-on");
    filter_table(sport, false);
  }
}

function in_array(array, item) {
  for (i=0;i<array.length;i++) {
    if (array[i] === item) {
      return true;
    }
  }
  return false;
}

function filter_table(query, hide) {
  var table, tr, td, i;
  table = document.getElementById("events_table");
  tr = table.getElementsByTagName("tr");

  // Loop through all table rows, and hide those who don't match the search query
  for (i = 0; i < tr.length; i++) {
    td = tr[i].getElementsByTagName("td")[0];
    if (td) {
      if (td.innerHTML.toUpperCase().indexOf(query.toUpperCase()) > -1) {
        if (hide) {
          tr[i].style.display = "none";
        } else {
          tr[i].style.display = "";
        }
      } 
    } 
  }
}
