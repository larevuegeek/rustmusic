

function durationToMinutes(totalSeconds: number): string {
    const minutes: number = Math.floor(totalSeconds / 60);
    const seconds: number = Math.floor(totalSeconds % 60);
    const formattedMinutes: string = minutes < 10 ? "0" + minutes : minutes.toString();
    const formattedSeconds: string = seconds < 10 ? "0" + seconds : seconds.toString();

    return `${formattedMinutes}:${formattedSeconds}`;
}

function formatTime(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
}

function dateToYear(string: string): string {

    let year = String("");

    const match = string.match(/\b(\d{4})\b/);
    if (match) {
      const y = Number(match[1]);
      year = (y >= 1000 && y <= 2999) ? String(y) : "";
    }

    return year;
}

export { durationToMinutes, formatTime, dateToYear };