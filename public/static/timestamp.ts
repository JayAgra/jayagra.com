"use strict";

/* Timestamp Handling */

function parseTimestamp(timestamp: number): Date {
    if (timestamp < 0) {
        throw new Error("Timestamp cannot be negative!");
    }
    
    const ms = timestamp < 1e12 ? timestamp * 1000 : timestamp;
    
    return new Date(ms);
}

function createTimestamp(dateTime: Date, zone: string, unit: string): number {
    const offset = Number(zone) * 1000;

    const adjustedMs = dateTime.getTime() + offset;

    return unit == "s" ? adjustedMs / 1000 : adjustedMs;    
}


/* Offset Handling */

interface readableTime {
    positive: boolean;
    years: number;
    days: number;
    hours: number;
    minutes: number;
    seconds: number;
}

function parseOffset(offset: number, unit: string): readableTime {
    const positive = offset > 0;
    offset = Math.abs(offset);

    const msAdjust = unit == "ms";

    const year = msAdjust ? 31536000000 : 31536000;
    const years = Math.floor(offset / year);
    offset %= year;

    const day = msAdjust ? 86400000 : 86400;
    const days = Math.floor(offset / day);
    offset %= day;

    const hour = msAdjust ? 3600000 : 3600;
    const hours = Math.floor(offset / hour);
    offset %= hour;

    const minute = msAdjust ? 60000 : 60;
    const minutes = Math.floor(offset / minute);
    offset %= minute;

    const second = msAdjust ? 1000 : 1;
    const seconds = Math.floor(offset / second);

    return {
        positive,
        years,
        days,
        hours,
        minutes,
        seconds
    }
}

function createOffset(years: number, days: number, hours: number, minutes: number, seconds: number, unit: string): number {
    const msAdjust = unit == "ms";
    var result = 0;

    const year = msAdjust ? 31536000000 : 31536000;
    result += Math.abs(years) * year;

    const day = msAdjust ? 86400000 : 86400;
    result += Math.abs(days) * day;

    const hour = msAdjust ? 3600000 : 3600;
    result += Math.abs(hours) * hour;

    const minute = msAdjust ? 60000 : 60;
    result += Math.abs(minutes) * minute;

    const second = msAdjust ? 1000 : 1;
    result += Math.abs(seconds) * second;

    return result;
}


/* Function Caller */

var timestampInput = (document.getElementById("timestampInput") as HTMLInputElement);
var timestampOutGmt = (document.getElementById("timestampOutGmt") as HTMLSpanElement);
var timestampOutLocal = (document.getElementById("timestampOutLocal") as HTMLSpanElement);
var timestampOutRelative = (document.getElementById("timestampOutRelative") as HTMLSpanElement);

var dateTime = (document.getElementById("dateTimeInput") as HTMLInputElement);
var timeZone = (document.getElementById("timeZoneInput") as HTMLSelectElement);
var timestampOutStamp = (document.getElementById("timestampOutStamp") as HTMLSpanElement);
var timestampOutSecStamp = (document.getElementById("timestampOutOtherStamp") as HTMLSpanElement);

var offsetInput = (document.getElementById("offsetInput") as HTMLInputElement);
var offsetInUnit = (document.getElementById("offsetInUnit") as HTMLSelectElement);
var offsetOutString = (document.getElementById("offsetOutString") as HTMLSpanElement);

var yearsInput = (document.getElementById("yearsInput") as HTMLInputElement);
var daysInput = (document.getElementById("daysInput") as HTMLInputElement);
var hoursInput = (document.getElementById("hoursInput") as HTMLInputElement);
var minutesInput = (document.getElementById("minutesInput") as HTMLInputElement);
var secondsInput = (document.getElementById("secondsInput") as HTMLInputElement);
var offsetOutStamp = (document.getElementById("offsetOutStamp") as HTMLInputElement);
var offsetOutOtherStamp = (document.getElementById("offsetOutOtherStamp") as HTMLInputElement);

// wrap this in try catch
function run(operation: string) {
    switch (operation) {
        case "parseTimestamp": {
            const strings = printDate(parseTimestamp(Number(timestampInput.value)));

            timestampOutGmt.innerText = strings.gmt;
            timestampOutLocal.innerText = strings.local;
            timestampOutRelative.innerText = strings.relative;

            break;
        }

        case "createTimestamp": {
            if (!dateTime.value) throw new Error("Bad date/time input");

            const strings = printTimestamp(createTimestamp(new Date(dateTime.value), timeZone.value, "ms"));

            timestampOutStamp.innerText = String(strings.milliseconds);
            timestampOutSecStamp.innerText = String(strings.seconds);

            break;
        }

        case "parseOffset": {
            const strings = readableOffset(parseOffset(Number(offsetInput.value), offsetInUnit.value));

            offsetOutString.innerText = strings;

            break;
        }

        case "createOffset": {
            const strings = printTimestamp(
                createOffset(
                    Number(yearsInput.value),
                    Number(daysInput.value),
                    Number(hoursInput.value),
                    Number(minutesInput.value),
                    Number(secondsInput.value),
                    "ms"
                )
            );

            offsetOutStamp.innerText = String(strings.milliseconds);
            offsetOutOtherStamp.innerText = String(strings.seconds);

            break;
        }
        
        default:
            break;
    }
}

setInterval(() => {
    run("parseTimestamp");
    run("createTimestamp");
    run("parseOffset");
    run("createOffset");
}, 20); 

/* Output/Prettyprint */

interface dateStrings {
    gmt: string,
    local: string,
    relative: string
}

function readableOffset(offset: readableTime): string {
    const parts: string[] = [];
    const add = (value: number, unit: string) => {
        if (value > 0) {
            parts.push(`${value} ${unit}${value != 1 ? "s" : ""}`);
        }
    }
    add(offset.years, "year"); add(offset.days, "day");
    add(offset.hours, "hour"); add(offset.minutes, "minute"); add(offset.seconds, "second");

    if (parts.length == 0) return "Now";

    const formatted = 
        parts.length == 1 ?
            parts[0] :
            parts.length == 2 ?
                `${parts[0]} and ${parts[1]}` :
                `${parts.slice(0, -1).join(", ")}, and ${parts[parts.length - 1]}`;

    return offset.positive ? `In ${formatted}` : `${formatted} ago`;
}

function printDate(date: Date): dateStrings {
    const gmt = date.toUTCString();
    const local = date.toLocaleString();

    const now = Date.now();
    const relative = readableOffset(parseOffset(date.getTime() - now, "ms"));

    return {
        gmt,
        local,
        relative
    }
}

interface timestampFormats {
    milliseconds: number;
    seconds: number;
}

function printTimestamp(milliseconds: number): timestampFormats {
    return {
        milliseconds,
        seconds: milliseconds / 1000
    }
}

function printOffset(milliseconds: number): timestampFormats {
    return {
        milliseconds,
        seconds: milliseconds / 1000
    }
}
