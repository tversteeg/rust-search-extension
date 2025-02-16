describe("CrateDocSearchManager", function() {
    before(function() {
        this.crateName = "matches";
        this.crateVersion = "0.1.8";
        this.searchIndex = {
            "matches": {
                "doc": "",
                "items": [[14, "matches", "matches", "Check if an expression matches a refutable pattern.", null, null], [14, "assert_matches", "", "Assert that an expression matches a refutable pattern.", null, null], [14, "debug_assert_matches", "", "Assert that an expression matches a refutable pattern using debug assertions.", null, null]],
                "paths": []
            }
        };
    });
    
    after(function() {
        CrateDocSearchManager.removeCrate(this.crateName);
    });

    describe("crates", function() {
        it("getCrates()", function() {
            CrateDocSearchManager.getCrates().should.deep.equal({});
        });
        it("addCrate()", function() {
            CrateDocSearchManager.addCrate(this.crateName, this.crateVersion, this.searchIndex);
            let crates = CrateDocSearchManager.getCrates();
            Object.keys(crates).should.contains(this.crateName);
        });
        it("getSearchIndex()", function() {
            let searchIndex = CrateDocSearchManager.getCrateSearchIndex(this.crateName);
            searchIndex.should.deep.equal(this.searchIndex);
        });
        it("removeCrate()", function() {
            CrateDocSearchManager.removeCrate(this.crateName);
            CrateDocSearchManager.getCrates().should.deep.equal({});
        });
    });

    describe("search", function() {
        let manager = new CrateDocSearchManager();
        [["@match", 2], ["@matches", 1], ["@matches m", 5], ["@matches z", 1]]
            .forEach(function([keyword, len]) {
                it(`"${keyword}" search()`, function() {
                    CrateDocSearchManager.addCrate(this.crateName, this.crateVersion, this.searchIndex);
                    let result = manager.search(keyword);
                    result.should.have.lengthOf(len);
                });
            });
    });
});