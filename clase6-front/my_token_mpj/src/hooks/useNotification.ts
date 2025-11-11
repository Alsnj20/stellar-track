import { use } from "react";
import {
  NotificationContext,
  NotificationContextType,
} from "../providers/NotificationProvider";

// Return a functions to show notifications (success, error, info, etc.)
export const useNotification = (): NotificationContextType => {
  const context = use(NotificationContext);
  if (!context) {
    throw new Error(
      "useNotification must be used within a NotificationProvider",
    );
  }
  return context;
};
