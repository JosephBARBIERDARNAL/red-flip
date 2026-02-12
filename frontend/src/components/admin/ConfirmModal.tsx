import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTimes, faSpinner } from "@fortawesome/free-solid-svg-icons";

interface ConfirmModalProps {
  title: string;
  message: string;
  confirmText: string;
  confirmColor?: string;
  onConfirm: () => void;
  onCancel: () => void;
  loading?: boolean;
  showReasonInput?: boolean;
  reason?: string;
  onReasonChange?: (reason: string) => void;
}

export default function ConfirmModal({
  title,
  message,
  confirmText,
  confirmColor = "bg-red-600",
  onConfirm,
  onCancel,
  loading = false,
  showReasonInput = false,
  reason = "",
  onReasonChange,
}: ConfirmModalProps) {
  const handleConfirm = () => {
    if (showReasonInput && !reason.trim()) {
      alert("Please provide a reason");
      return;
    }
    onConfirm();
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg shadow-xl max-w-md w-full">
        <div className="flex items-center justify-between p-4 border-b border-brand-200">
          <h3 className="font-serif text-xl font-bold text-brand-800">
            {title}
          </h3>
          <button
            onClick={onCancel}
            className="text-brand-400 hover:text-brand-600"
            disabled={loading}
          >
            <FontAwesomeIcon icon={faTimes} />
          </button>
        </div>

        <div className="p-4 space-y-4">
          <p className="text-brand-700">{message}</p>

          {showReasonInput && (
            <div>
              <label className="block text-sm font-medium text-brand-700 mb-1">
                Reason <span className="text-red-500">*</span>
              </label>
              <textarea
                value={reason}
                onChange={(e) => onReasonChange?.(e.target.value)}
                className="w-full px-3 py-2 border border-brand-300 rounded-md focus:outline-none focus:ring-2 focus:ring-brand-500"
                rows={3}
                placeholder="Enter reason for ban..."
                maxLength={500}
                required
              />
              <p className="text-xs text-brand-500 mt-1">
                {reason.length}/500 characters
              </p>
            </div>
          )}

          <div className="flex gap-3 pt-2">
            <button
              type="button"
              onClick={onCancel}
              className="flex-1 px-4 py-2 border border-brand-300 text-brand-700 rounded-md hover:bg-brand-50"
              disabled={loading}
            >
              Cancel
            </button>
            <button
              type="button"
              onClick={handleConfirm}
              className={`flex-1 px-4 py-2 ${confirmColor} text-white rounded-md hover:opacity-90 disabled:opacity-50`}
              disabled={loading}
            >
              {loading ? (
                <FontAwesomeIcon icon={faSpinner} spin />
              ) : (
                confirmText
              )}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
