class Iteration {
    void visit() {
        for (DriverSplitRunnerFactory driverRunnerFactory : driverRunnerFactoriesWithTaskLifeCycle) {
            run(driverRunnerFactory);
        }
        for (String value : values) use(value);
        for (String value : loadValues(firstVeryLongArgumentName, secondVeryLongArgumentName, thirdVeryLongArgumentName)) {
            use(value);
        }
    }
}
