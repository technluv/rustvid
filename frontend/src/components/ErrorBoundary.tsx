import React, { Component, ErrorInfo, ReactNode } from 'react';
import { AlertTriangleIcon, RefreshCwIcon } from 'lucide-react';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error?: Error;
  errorInfo?: ErrorInfo;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Error caught by boundary:', error, errorInfo);
    this.setState({
      error,
      errorInfo,
    });
  }

  handleReset = () => {
    this.setState({ hasError: false, error: undefined, errorInfo: undefined });
  };

  handleReload = () => {
    window.location.reload();
  };

  render() {
    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback;
      }

      return (
        <div 
          className="flex items-center justify-center min-h-[400px] bg-gray-900 text-white p-8"
          role="alert"
          aria-live="assertive"
        >
          <div className="text-center max-w-md">
            <AlertTriangleIcon 
              className="w-16 h-16 text-red-500 mx-auto mb-4" 
              aria-hidden="true"
            />
            <h2 className="text-2xl font-bold mb-4">Oops! Something went wrong</h2>
            <p className="text-gray-300 mb-6">
              An unexpected error occurred in the video editor. This might be due to a 
              temporary issue or corrupted data.
            </p>
            
            {/* Error details (collapsed by default) */}
            <details className="text-left mb-6 bg-gray-800 rounded p-4">
              <summary className="cursor-pointer font-semibold text-red-400 hover:text-red-300">
                Technical Details
              </summary>
              <div className="mt-2 text-sm font-mono text-gray-400">
                <div className="mb-2">
                  <strong>Error:</strong> {this.state.error?.message}
                </div>
                {this.state.error?.stack && (
                  <div>
                    <strong>Stack:</strong>
                    <pre className="mt-1 text-xs bg-gray-900 p-2 rounded overflow-auto max-h-32">
                      {this.state.error.stack}
                    </pre>
                  </div>
                )}
              </div>
            </details>

            <div className="flex gap-4 justify-center">
              <button
                onClick={this.handleReset}
                className="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500"
                aria-label="Try to recover from error"
              >
                <RefreshCwIcon className="w-4 h-4" aria-hidden="true" />
                Try Again
              </button>
              
              <button
                onClick={this.handleReload}
                className="px-4 py-2 bg-gray-600 hover:bg-gray-700 rounded transition-colors focus:outline-none focus:ring-2 focus:ring-gray-500"
                aria-label="Reload the entire application"
              >
                Reload App
              </button>
            </div>

            <div className="mt-6 text-sm text-gray-400">
              <p>If this problem persists, try:</p>
              <ul className="list-disc list-inside mt-2 space-y-1">
                <li>Refreshing the page</li>
                <li>Clearing your browser cache</li>
                <li>Using a different browser</li>
                <li>Checking your internet connection</li>
              </ul>
            </div>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}