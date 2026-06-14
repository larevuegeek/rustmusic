export type QueueTrack = {
  queueId: string;
  profilId: number;
  path: string;
  title: string;
  artist?: string;
  duration?: number;
  cover?: string;
  position: number;
};