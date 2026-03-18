import { writable } from "svelte/store";

export interface PrayerTime {
  name: string;
  time: string;
  passed: boolean;
  is_next: boolean;
  icon: string;
}

export interface PrayerTimesResult {
  prayers: PrayerTime[];
  sunrise: string;
  sunset: string;
  imsak: string;
  midnight: string;
  firstthird: string;
  lastthird: string;
  hijri_date: string;
  gregorian_date: string;
  next_prayer: string;
  next_prayer_time: string;
  time_remaining: string;
  method_name: string;
}

export interface PrayerConfig {
  city: string;
  country: string;
  method: number;
  school: number;
  notify_before_mins: number;
}

export const prayerData = writable<PrayerTimesResult | null>(null);
export const config = writable<PrayerConfig>({
  city: "Jakarta",
  country: "ID",
  method: 20,
  school: 0,
  notify_before_mins: 1,
});
export const loading = writable(true);
export const error = writable<string | null>(null);
export const showSettings = writable(false);
