test_that("multiplication works", {
  expect_equal(2 * 2, 4)
})

test_that("greeting_n_times", {
  n <- 10
  expect_equal(
    greeting_n_times(),
    paste(rep("Hello world!\n", n), collapse = "")
  )
})