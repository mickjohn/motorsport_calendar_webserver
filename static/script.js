var allButton;
var filterButtons;
var numberOfButtons;
var numberOfButtonsOn;

function showAll(e) {
  numberOfButtonsOn = numberOfButtons;
  for (var filter_button of filterButtons) {
    var sport = filter_button.innerText.replace(" ", "_");
    var class_name = sport + "_filter_button";
    if (!in_array(filter_button.classList, class_name)) {
      filter_button.classList.add(class_name);
      showSport(sport);
    }
  }
}

function showSport(sportName) { 
  var class_name = sportName + "_event_list_item";
  var list_items = document.getElementsByClassName(class_name);
  for (var list_item of list_items) {
    list_item.classList.remove("event_list_item_hidden");
  }
}

function hideSport(sportName) {
  var class_name = sportName + "_event_list_item";
  var list_items = document.getElementsByClassName(class_name);
  for (var list_item of list_items) {
    list_item.classList.add("event_list_item_hidden");
  }
}

function filterButtonClicked(e) {
  var button = e.target;
  var sport = button.innerText.replace(" ", "_");
  var class_name = sport + "_filter_button";
  var button_is_on = in_array(button.classList, class_name);

  if (button_is_on && numberOfButtonsOn === numberOfButtons) {
    // Hide all except this
    numberOfButtonsOn = 1;
    for (var filterButton of filterButtons) {
      var otherSport = filterButton.innerText.replace(" ", "_");
      var otherClassName = otherSport + "_filter_button";
      if (otherSport !== sport && otherSport !== 'all') {
        hideSport(otherSport);
        filterButton.classList.remove(otherClassName);
      }
    }
  } else if (button_is_on) {
    // Turn it off
    numberOfButtonsOn -= 1;
    hideSport(sport)
    button.classList.remove(class_name);
  } else {
    // Turn it on
    numberOfButtonsOn += 1;
    showSport(sport)
    button.classList.add(class_name);
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

document.addEventListener('DOMContentLoaded', function () {
  // Add click listener to the filter buttons
  allButton = document.getElementById('all_filter_button');
  filterButtons = document.getElementsByClassName('filter_button');
  numberOfButtons = filterButtons.length;
  numberOfButtonsOn = filterButtons.length;

  allButton.addEventListener('click', showAll);

  for (var element of filterButtons) {
    element.addEventListener('click', filterButtonClicked);
  }
});
