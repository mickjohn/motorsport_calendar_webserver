
// Use these tests to validate the UI, not the data.
// To validate the data I can always use a script and
// parse the XML and check the data

var url = "http://localhost:8080/"

function isDesktopLayout() {
  return browser.driver.manage().window().getSize().then( function(size){ 
    return size.width > 1023;
  });
}

function getEventRows() {
  return element(by.id('events_table')).$('tbody').$$('tr').filter( function(row, index) {
    return index != 0;
  });
}

function getEventRowsWithoutSessionRows() {
  return getEventRows().filter(function(row, index) {
    row.getAttribute('class').then(function (row_class) {
      return row_class != "sessions-row" && row_class != "sessions-row-hidden";
    });
  });
}

function getFilterButtonsWithoutAll() {
  if (isDesktopLayout()) {
    var nav_class = 'sidenavFilterButtons';
  } else {
    var nav_class = 'centre_nav';
  }

  filter_buttons = element(by.id(nav_class)).$$('a').filter( function(elem, index) {
    return elem.getText().then(function (text) {
      return text != 'ALL';
    });
  });
  return filter_buttons;
}

function getAllButton() {
  if (isDesktopLayout()) {
    var button_id = 'all_filter';
  } else {
    var button_id = 'all_filter';
  }

  all_button = element(by.id(button_id));
  return all_button;
}

describe('The page', function() {
  it('loads', function() {
    browser.waitForAngularEnabled(false);
    browser.get(url);
    expect(element(by.id('events_table')).getTagName()).toBe('table');
    expect($$('h1').get(1).getText()).toBe('Upcoming Motorsport Events');
  });
});

describe('events table', function() {
  it('has a table', function() {
    browser.waitForAngularEnabled(false);
    browser.get(url);
    rows = element(by.id("events_table")).$('tbody').$$('tr');
    expect(rows.get(1).$('td').getText()).toBe('+');
  });
});

describe('The rows in the events table', function() {
  it('expand when click', function() {
    browser.waitForAngularEnabled(false);
    browser.get(url);
    rows = getEventRows();

    // First row is the row with the event.
    // Second row is the row with the hidden session.
    // Clicking the event row should reveal the session row beneath.
    first_event = rows.get(0);
    first_session = rows.get(1);

    expect(first_session.getAttribute('class')).toEqual('sessions-row-hidden');
    first_event.click();
    expect(first_session.getAttribute('class')).toEqual('sessions-row');
  });

  it('collapse if expanded when clicked', function() {
    browser.waitForAngularEnabled(false);
    browser.get(url);
    rows = getEventRows();

    // First row is the row with the event.
    // Second row is the row with the hidden session.
    // Clicking the event row should reveal the session row beneath.
    first_event = rows.get(0);
    first_session = rows.get(1);
    first_event.click(); // Click to expand it

    expect(first_session.getAttribute('class')).toEqual('sessions-row');
    first_event.click();
    expect(first_session.getAttribute('class')).toEqual('sessions-row-hidden');
  });

  it('are hidden when the filter is applied', function() {
    browser.driver.manage().window().maximize();
    browser.waitForAngularEnabled(false);
    browser.get(url);

    event_rows = getEventRowsWithoutSessionRows();
    filter_buttons = getFilterButtonsWithoutAll();
    all_button = getAllButton();

    filter_buttons.each(function (filter_button) {
      filter_button.click();
      filter_button.getText().then(function (text) {
        event_rows.each(function (row) {
          row.all(By.tagName('td')).get(1).getText(function (row_sport) {
            if ( row_sport == text ) {
              expect(row.isDisplayed()).toBeTruthy();
            }
          });
        });
      });
      //Click the all button to reset the filters.
      all_button.click();
    });
    expect(true).toBe(true);
  });

});

// TEST CASES:
// TODO
// #1 First test that there are enough events,sessions and event types facilitate a test
//
// 
// #2 Row expands and when clicked
//
// 
// #3 Row collapses when clicked
//
// 
// #4 Rows are hidden when filter is clicked, except for the sport type of the filter
//
// TODO
// #5 Clicking another filter adds the sport types of that filter to the table
//
// TODO
// #6 Clicking all shows all rows
