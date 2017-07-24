
var globals = {
  // right_triangle: '&#x25BA',
  // down_triangle: '&#x25BC'
  right_triangle: '<b>+</b>',
  down_triangle: '<b>-</b>'
};

function hide_all_sports_but_one(sport_type) {
  var anchors = document.getElementById('mySidenav').getElementsByTagName("A");
  var sports = [];
  for (var i=0;i<anchors.length;i++) {
    var current_sport_type = anchors[i].text;
    // if (current_sport_type != sport_type && current_sport_type != "ALL") {
    if (current_sport_type != sport_type) {
      filter_table(current_sport_type, true);
    }
  }
}

function show_all() {
  var all_button = document.getElementById('all_filter');
  var anchors = document.getElementById('mySidenav').getElementsByTagName("A");
  var sports = [];
  for (var i=0;i<anchors.length;i++) {
    var current_sport_type = anchors[i].text;
    if (current_sport_type == "ALL") {
      all_button.classList.add('filter-button-on');
      all_button.classList.remove('filter-button-off');
    } else {
      var button = anchors[i];
      button.classList.add("filter-button-off");
      button.classList.remove("filter-button-on");
      filter_table(current_sport_type, false);
    }
  }
}

function toggle_button_clicked(button, sport_type) {
  var class_list = button.classList;
  var all_button = document.getElementById('all_filter');
  var all_button_on = in_array(all_button.classList, 'filter-button-on');

  if (!all_button_on && sport_type == "ALL") {
    show_all();
    return;
  } else if ( sport_type == "ALL" ) {
    return;
  }

  // If it's on turn it off, and hide it's events
  if (in_array(class_list, 'filter-button-on')) {
    button.classList.add("filter-button-off");
    button.classList.remove("filter-button-on");
    filter_table(sport_type, true);
  } else {
    button.classList.remove("filter-button-off");
    button.classList.add("filter-button-on");
    if (all_button_on) {
      hide_all_sports_but_one(sport_type);
      all_button.classList.remove('filter-button-on');
    }
    else {
      filter_table(sport_type, false);
    }
  }
}

function toggle_side_nav() {
  var side_nav = document.getElementById('mySidenav');
  var nav_button = document.getElementById('nav-button');
  if (side_nav.style.display == 'none' || side_nav.style.display == '') {
    side_nav.style.display = 'block';
    nav_button.style.marginLeft = '220px';
  } else {
    nav_button.style.marginLeft = '0';
    side_nav.style.display = 'none';
  }
}

function hide_session_row(session_row) {
  if(!in_array(session_row.classList, 'session-row-hidden')) {
    session_row.classList.add('sessions-row-hidden');
  }
  session_row.classList.remove('sessions-row');
}

function show_session_row(session_row) {
  session_row.classList.remove('sessions-row-hidden');
  if(!in_array(session_row.classList, 'session-row')) {
    session_row.classList.add('sessions-row');
  }
}

function toggle_session_row(event_row) {
  var table = document.getElementById('events_table');
  var session_row = table.rows[event_row.rowIndex + 1];

  if (in_array(session_row.classList, 'sessions-row')) {
    hide_session_row(session_row);
    set_event_row_triangle(event_row, globals.right_triangle);
  } else {
    show_session_row(session_row);
    set_event_row_triangle(event_row, globals.down_triangle);
  }
}

function set_event_row_triangle(event_row, new_triangle) {
  var triangle_element = event_row.getElementsByTagName("td")[0];
  triangle_element.innerHTML = new_triangle;
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
  var table = document.getElementById("events_table");
  var table_rows = table.rows;

  // Loop through all table rows, and hide those who don't match the search query
  for (var i = 0; i < table_rows.length; i++) {
    var current_row = table_rows[i];
    var td = current_row.getElementsByTagName("td")[1];
    if (td) {
      if (td.innerHTML.toUpperCase().indexOf(query.toUpperCase()) > -1) {
        if (hide) {
          /* If the sessions for this row are expanded, hide them */
          /* First get the 'session_row'. This is the current row + 1 */
          var session_row = table_rows[current_row.rowIndex + 1];
          /* We also need set the triangle */
          set_event_row_triangle(current_row, globals.right_triangle);
          /* Then hide the session row */
          hide_session_row(session_row);
          /* And then hide the row */
          current_row.style.display = "none";
        } else {
          current_row.style.display = "";
        }
      } 
    } 
  }
}

// Filter out duplicate items in an array
// http://stackoverflow.com/questions/11688692/most-elegant-way-to-create-a-list-of-unique-items-in-javascript

// http://stackoverflow.com/questions/11309859/css-media-queries-and-javascript-window-width-do-not-match
function viewport() {
  var e = window, a = 'inner';
  if (!('innerWidth' in window )) {
    a = 'client';
    e = document.documentElement || document.body;
  }
  return { width : e[ a+'Width' ] , height : e[ a+'Height' ] };
}

window.onresize = function(event) {
  var v = viewport();
  if (v.width >= 1023) {
    var side_nav = document.getElementById('mySidenav');
    side_nav.style.display = 'block';
  } else {
    var side_nav = document.getElementById('mySidenav');
    side_nav.style.display = 'none';
  }
};
