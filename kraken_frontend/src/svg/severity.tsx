import React from "react";
import "../index.css";

type SeverityIconProps = {
    severity?: "ok" | "low" | "medium" | "high" | "critical";
};

export default function SeverityIcon(props: SeverityIconProps) {
    const { severity } = props;
    return (
        <div className={"icon"} {...props}>
            <svg
                width="800px"
                height="800px"
                className={
                    severity === "ok"
                        ? "severity-icon-ok"
                        : severity === "low"
                          ? "severity-icon-low"
                          : severity === "medium"
                            ? "severity-icon-medium"
                            : severity === "high"
                              ? "severity-icon-high"
                              : severity === "critical"
                                ? "severity-icon-critical"
                                : "neon"
                }
                viewBox="0 0 24 24"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path d="M12 9V14" stroke="#292D32" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
                <path
                    d="M12.0001 21.41H5.94005C2.47005 21.41 1.02005 18.93 2.70005 15.9L5.82006 10.28L8.76006 5.00003C10.5401 1.79003 13.4601 1.79003 15.2401 5.00003L18.1801 10.29L21.3001 15.91C22.9801 18.94 21.5201 21.42 18.0601 21.42H12.0001V21.41Z"
                    stroke="#292D32"
                    strokeWidth="1.5"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                />
                <path
                    d="M11.9945 17H12.0035"
                    stroke="#292D32"
                    strokeWidth="2"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                />
            </svg>
        </div>
    );
}
