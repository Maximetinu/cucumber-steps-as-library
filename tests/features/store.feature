Feature: store

  Scenario: 1
    When I store 1
    Then I see 1

  Scenario: 1 and 2
    Given I stored 1
    When I store 2
    Then I see 1 and 2

  Scenario: 1, 2 and 3
    Given I stored 1
    Given I stored 2
    When I store 3
    Then I see 1, 2 and 3