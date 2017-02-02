
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

function toggle_sessions(event_row, session_table_id) {
  var session_row = document.getElementById(session_table_id);
  var triangle = event_row.getElementsByTagName("td")[0];

  var class_list = session_row.classList;
  if (in_array(class_list, 'sessions-row')) {
    session_row.classList.add('sessions-row-hidden');
    session_row.classList.remove('sessions-row');
    triangle.innerHTML = '&#x25BA';
  } else {
    session_row.classList.remove('sessions-row-hidden');
    session_row.classList.add('sessions-row');
    triangle.innerHTML = '&#x25BC';
  }
}

function in_array(array, item) {
  for (var i=0;i<array.length;i++) {
    if (array[i] === item) {
      return true;
    }
  }
  return false;
}

function filter_table(query, hide) {
  var table, tr, td;
  table = document.getElementById("events_table");
  tr = table.getElementsByTagName("tr");

  // Loop through all table rows, and hide those who don't match the search query
  for (var i = 0; i < tr.length; i++) {
    td = tr[i].getElementsByTagName("td")[1];
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

function get_list_of_sport_names() {
  var table = document.getElementById("events_table");
  var rows = table.getElementsByTagName("tr");
  var sports = [];

  for (var i = 0; i < rows.length; i++) {
    var class_list = rows[i].classList;
    // Only want to check events_table_row.
    if (in_array(class_list, 'events_table_row')) {
      var td = rows[i].getElementsByTagName("td")[1];
      if (td) {
        sports.push(td.innerHTML.trim());
      }
    }
  }
  return sports;
}

//http://stackoverflow.com/questions/11688692/most-elegant-way-to-create-a-list-of-unique-items-in-javascript
function unique(arr) {
  var u = {}, a = [];
  for(var i = 0, l = arr.length; i < l; ++i){
    if(!u.hasOwnProperty(arr[i])) {
      a.push(arr[i]);
      u[arr[i]] = 1;
    }
  }
  return a;
}

// Generate the filter buttons based on the sport names
function generate_filter_buttons() {
  var sports = unique(get_list_of_sport_names());
  var sidenav_div = document.getElementById("mySidenav");

  for(var i = 0; i < sports.length; i++) {
    var sport_name = sports[i];
    var button = document.createElement("A");
    var button_text = document.createTextNode(sport_name);
    button.appendChild(button_text);
    button.setAttribute("class", "filter-button filter-button-on");
    button.setAttribute("href", "#");
    button.setAttribute("onclick", "toggle_button_clicked(this,'" + sport_name + "')");
    sidenav_div.appendChild(button);
  }
}
