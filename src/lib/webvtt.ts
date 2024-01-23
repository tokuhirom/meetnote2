export class Caption {
    constructor(public startTime: string, public endTime: string, public text: string) {
        this.startTime = startTime;
        this.endTime = endTime;
        this.text = text;
    }

    parseStartTimeMillis(): number {
        const timeParts: string[] = this.startTime.split(':');
        const hour: number = parseInt(timeParts[0]);
        const minute: number = parseInt(timeParts[1]);
        const secondParts: string[] = timeParts[2].split('.');
        const second: number = parseInt(secondParts[0]);
        const millisec: number = parseInt(secondParts[1]);
        return millisec + second * 1000 + minute * 60 * 1000 + hour * 60 * 60 * 1000;
    }
}

export function parseWebVTT(webvtt: string): Caption[] {
    const captions: Caption[] = [];
    const timePattern = /(\d{2}:\d{2}:\d{2}\.\d{3}) --> (\d{2}:\d{2}:\d{2}\.\d{3})/;

    let currentStartTime = '';
    let currentEndTime = '';
    let currentText = '';

    webvtt.split('\n').forEach((line) => {
        const match = line.match(timePattern);
        if (match) {
            currentStartTime = match[1];
            currentEndTime = match[2];
        } else if (line !== '' && line !== 'WEBVTT') {
            if (currentText !== '') {
                currentText += '\n';
            }
            currentText += line.trim();
        } else if (line === '' && currentText !== '') {
            captions.push(new Caption(currentStartTime, currentEndTime, currentText));
            currentText = '';
        }
    });

    // Handle case where the last caption doesn't end with a newline
    if (currentText !== '') {
        captions.push(new Caption(currentStartTime, currentEndTime, currentText));
    }

    return captions;
}

export function compactWebVtt(captions: Caption[]): Caption[] {
    const compactedCaptions: Caption[] = [];
    let previousCaption: Caption | null = null;

    for (const caption of captions) {
        if (previousCaption && caption.text === previousCaption.text) {
            // Update only the end time if the text is the same
            previousCaption.endTime = caption.endTime;
        } else {
            // Add a new caption if the text is different
            previousCaption = new Caption(caption.startTime, caption.endTime, caption.text);
            compactedCaptions.push(previousCaption);
        }
    }

    return compactedCaptions;
}
