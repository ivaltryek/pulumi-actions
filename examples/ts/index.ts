import * as pulumi from "@pulumi/pulumi";
import * as random from "@pulumi/random";

const string = new random.RandomString("random", {
    length: 16,
    overrideSpecial: "/@£$",
    special: false,
});

export const op = string.result;
